/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use near_sdk::AccountId;

    use crate::{Address, Contract, PlaceInput};

    fn get_contract() -> Contract {
        let bob: AccountId = "bob.near".parse().unwrap();
        Contract::init(bob, None)
    }

    fn add_places_to_contract(contract: &mut Contract) {
        contract.add_place(PlaceInput {
            name: "Attelier Restaurant".to_string(),
            address: Address {
                address: "Savassi".to_string(),
                country: "Brazil".to_string(),
                state_or_province: "Minas Gerais".to_string(),
                city: "Belo Horizonte".to_string(),
            },
            place_type: "restaurant".to_string(),
            description: "Um bom restaurante para visitar com a familia.".to_string(),
            pictures: vec![
                "https://cdn.vox-cdn.com/thumbor/5d_RtADj8ncnVqh-afV3mU-XQv0=/0x0:1600x1067/1200x900/filters:focal(672x406:928x662)/cdn.vox-cdn.com/uploads/chorus_image/image/57698831/51951042270_78ea1e8590_h.7.jpg".to_string(),
                "https://upload.wikimedia.org/wikipedia/commons/thumb/e/ef/Restaurant_N%C3%A4sinneula.jpg/640px-Restaurant_N%C3%A4sinneula.jpg".to_string()
            ],
        });
    }

    #[test]
    fn add_places_then_get_places() {
        let mut contract = get_contract();
        add_places_to_contract(&mut contract);

        let places = contract.get_places();
        let first_child = places.first().unwrap();

        assert_eq!(places.len(), 1);
        assert_eq!(first_child.id, 0);
    }
}