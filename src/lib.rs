use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct FidanBagis {
    vector_of_donations: Vec<Donation>,
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Donation {
    donator_id: u32,
    amount: u32,
    location_id: u32,
}


#[near_bindgen]
impl FidanBagis {
    pub fn get_donation(&mut self, _index: usize) -> Donation {
        return self.vector_of_donations[_index];
    }

    pub fn get_vector_of_donations(&mut self) -> Vec<Donation> {
        return self.vector_of_donations;
    }

    pub fn deposit_donation(&mut self, _donator: u32, _amount: u32, _location: u32) {
        let new_donation = Donation {
            donator_id: _donator,
            amount: _amount,
            location_id: _location
        };

        self.vector_of_donations.push(new_donation);
        let log_message = format!("New donation has pushed");
        env::log(log_message.as_bytes());
    }
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
