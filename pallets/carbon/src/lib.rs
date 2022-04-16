#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use scale_info::TypeInfo;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::{
        sp_runtime::traits::Hash,
        traits::{ Randomness, Currency, tokens::ExistenceRequirement },
        transactional
    };
    use sp_io::hashing::blake2_128;

    #[cfg(feature = "std")]
    use frame_support::serde::{Deserialize, Serialize};

    // Struct to hold Credit information.
    type AccountOf<T> = <T as frame_system::Config>::AccountId;

    // Struct for holding Credit information.
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    #[codec(mel_bound())]
    pub struct Credit<T: Config> {
        pub source: Source,
        pub serial_number: [u8; 256],
        pub retired: bool,
        pub owner: AccountOf<T>,
    }

    // Enum declaration for Source (3rd-party registries)
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub enum Source {
        VCS,
        GoldStandard,
        AmericanCarbonRegistry,
    }

    // ACTION #3: Implementation to handle Gender type in Kitty struct.

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types it depends on.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The Currency handler for the Kitties pallet.
        type Currency: Currency<Self::AccountId>;

        // ACTION #5: Specify the type for Randomness we want to specify for runtime.

        // ACTION #9: Add MaxKittyOwned constant
    }

    // Errors.
    #[pallet::error]
    pub enum Error<T> {
        // TODO Part III
    }

    // Events.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // TODO Part III
    }

    #[pallet::storage]
    #[pallet::getter(fn count_for_credits)]
    /// Keeps track of the number of Credits in existence.
    pub(super) type CountForCredits<T: Config> = StorageValue<_, u64, OptionQuery>;

    // ACTION #7: Remaining storage items.

    // TODO Part IV: Our pallet's genesis configuration.

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        // TODO Part III: create_kitty

        // TODO Part IV: set_price

        // TODO Part IV: transfer

        // TODO Part IV: buy_kitty

        // TODO Part IV: breed_kitty
    }

    //** Our helper functions.**//

    impl<T: Config> Pallet<T> {

        // ACTION #4: helper function for Kitty struct

        // TODO Part III: helper functions for dispatchable functions

        // ACTION #6: function to randomly generate DNA

        // TODO Part III: mint

        // TODO Part IV: transfer_kitty_to
    }
}