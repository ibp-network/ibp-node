#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::RuntimeDebug;
// Re-export pallet items so that they can be accessed from the crate namespace.
use frame_support::{
	pallet_prelude::*,
	traits::{Currency, ReservableCurrency},
};
use frame_system::pallet_prelude::*;
pub use pallet::*;
use scale_info::TypeInfo;

pub mod weights;
pub use weights::*;

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub enum ServiceType {
	RPC,
	BootNode,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct Service {
	id: u32,
	ty: ServiceType,
	name: BoundedVec<u8, ConstU32<64>>,
	url_path: BoundedVec<u8, ConstU32<32>>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct Member {
	id: u32,
	name: BoundedVec<u8, ConstU32<64>>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct MemberService {
	service_id: u32,
	member_id: u32,
	id: u32,
	name: BoundedVec<u8, ConstU32<64>>,
	address: BoundedVec<u8, ConstU32<128>>,
	port: u16,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct HealthCheck {
	member_service_id: u32,
	timestamp: u64,
	status: bool,
	response_time_ms: u32,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
		type Currency: ReservableCurrency<Self::AccountId>;
		#[pallet::constant]
		type HealthCheckReward: Get<u32>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ServiceRegistered {
			id: u32,
			name: BoundedVec<u8, ConstU32<64>>,
		},
		MemberRegistered {
			account_id: T::AccountId,
			id: u32,
			name: BoundedVec<u8, ConstU32<64>>,
		},
		MemberServiceRegistered {
			service_id: u32,
			member_id: u32,
			id: u32,
			name: BoundedVec<u8, ConstU32<64>>,
		},
		MonitorRegistered {
			who: T::AccountId,
			name: BoundedVec<u8, ConstU32<32>>,
		},
		HealthCheckSubmitted {
			member_service_name: BoundedVec<u8, ConstU32<64>>,
			monitor_name: BoundedVec<u8, ConstU32<32>>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		ServiceAlreadyRegistered,
		MemberAlreadyRegistered,
		InvalidMemberName,
		MemberServiceAlreadyRegistered,
		ServiceNotFound,
		MemberNotFound,
		InvalidIP4Address,
		InvalidPort,
		MonitorAlreadyRegistered,
		MemberServiceNotFound,
		MonitorNotFound,
	}

	#[pallet::storage]
	pub(super) type ServiceCount<T: Config> = StorageValue<_, u32>;

	#[pallet::storage]
	pub(super) type Services<T: Config> = StorageMap<_, Blake2_128Concat, u32, Service>;

	#[pallet::storage]
	pub(super) type MemberCount<T: Config> = StorageValue<_, u32>;

	#[pallet::storage]
	pub(super) type Members<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Member>;

	#[pallet::storage]
	pub(super) type MemberServiceCount<T: Config> = StorageValue<_, u32>;

	#[pallet::storage]
	pub(super) type MemberServices<T: Config> = StorageMap<_, Blake2_128Concat, u32, MemberService>;

	#[pallet::storage]
	pub(super) type Monitors<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<u8, ConstU32<32>>>;

	#[pallet::storage]
	pub(super) type HealthChecks<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		u32,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<HealthCheck, ConstU32<512>>,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::dummy_weight())]
		pub fn register_service(
			origin: OriginFor<T>,
			ty: ServiceType,
			name: BoundedVec<u8, ConstU32<64>>,
			url_path: BoundedVec<u8, ConstU32<32>>,
		) -> DispatchResult {
			ensure_root(origin)?;
			let id = match ServiceCount::<T>::get() {
				Some(count) => count,
				None => 0,
			};
			ServiceCount::<T>::set(Some(id + 1));
			ensure!(!Services::<T>::contains_key(&id), Error::<T>::ServiceAlreadyRegistered);
			let service = Service { id, ty, name: name.clone(), url_path };
			Services::<T>::insert(&id, service);
			Self::deposit_event(Event::ServiceRegistered { id, name });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::dummy_weight())]
		pub fn register_member(
			origin: OriginFor<T>,
			name: BoundedVec<u8, ConstU32<64>>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let id = match MemberCount::<T>::get() {
				Some(count) => count,
				None => 0,
			};
			MemberCount::<T>::set(Some(id + 1));
			ensure!(!Members::<T>::contains_key(&sender), Error::<T>::MemberAlreadyRegistered);
			ensure!(!name.is_empty(), Error::<T>::InvalidMemberName);
			let member = Member { id, name: name.clone() };
			Members::<T>::insert(&sender, member);
			Self::deposit_event(Event::MemberRegistered { account_id: sender.clone(), id, name });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::dummy_weight())]
		pub fn register_member_service(
			origin: OriginFor<T>,
			service_id: u32,
			name: BoundedVec<u8, ConstU32<64>>,
			address: BoundedVec<u8, ConstU32<128>>,
			port: u16,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let service = Services::<T>::get(&service_id).ok_or(Error::<T>::ServiceNotFound)?;
			let member = Members::<T>::get(&sender).ok_or(Error::<T>::MemberNotFound)?;
			let id = match MemberServiceCount::<T>::get() {
				Some(count) => count,
				None => 0,
			};
			MemberServiceCount::<T>::set(Some(id + 1));
			ensure!(
				!MemberServices::<T>::contains_key(&id),
				Error::<T>::MemberServiceAlreadyRegistered,
			);
			ensure!(!address.is_empty(), Error::<T>::InvalidIP4Address,);
			let member_service = MemberService {
				service_id: service.id,
				member_id: member.id,
				id,
				name: name.clone(),
				address: address.clone(),
				port,
			};
			MemberServices::<T>::insert(&id, member_service);
			Self::deposit_event(Event::MemberServiceRegistered {
				service_id: service.id,
				member_id: member.id,
				id,
				name: name.clone(),
			});
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::dummy_weight())]
		pub fn register_monitor(
			origin: OriginFor<T>,
			monitor: T::AccountId,
			name: BoundedVec<u8, ConstU32<32>>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(Members::<T>::contains_key(&sender), Error::<T>::MemberNotFound);
			ensure!(!Monitors::<T>::contains_key(&monitor), Error::<T>::MonitorAlreadyRegistered);
			Monitors::<T>::insert(&monitor, name.clone());
			Self::deposit_event(Event::MonitorRegistered { who: sender, name });
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::dummy_weight())]
		pub fn submit_health_check(
			origin: OriginFor<T>,
			member_service_id: u32,
			timestamp: u64,
			status: bool,
			response_time_ms: u32,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let member_service = MemberServices::<T>::get(&member_service_id)
				.ok_or(Error::<T>::MemberServiceNotFound)?;
			let monitor_name = Monitors::<T>::get(&sender).ok_or(Error::<T>::MonitorNotFound)?;
			let mut service_health_checks =
				match HealthChecks::<T>::get(&member_service_id, &sender) {
					Some(service_health_checks) => service_health_checks,
					None => BoundedVec::default(),
				};
			let health_check =
				HealthCheck { member_service_id, timestamp, status, response_time_ms };
			service_health_checks.try_push(health_check).unwrap();
			HealthChecks::<T>::set(&member_service_id, &sender, Some(service_health_checks));
			let reward: BalanceOf<T> = T::HealthCheckReward::get().into();
			T::Currency::deposit_creating(&sender, reward);
			Self::deposit_event(Event::HealthCheckSubmitted {
				member_service_name: member_service.name.clone(),
				monitor_name: monitor_name.clone(),
			});
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::zero_weight())]
		pub fn mint(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let reward: BalanceOf<T> = T::HealthCheckReward::get().into();
			T::Currency::deposit_creating(&sender, reward);
			Ok(())
		}
	}
}
