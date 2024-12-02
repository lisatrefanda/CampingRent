use ic_cdk::api::{call, trap};
use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CandidType, Serialize, Deserialize, Debug)]
struct CampingGear {
    id: u64,
    name: String,
    description: String,
    daily_rate: u64, // Harga sewa per hari
    quantity: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
struct RentalTransaction {
    user: Principal,
    gear_id: u64,
    days: u64,
    total_cost: u64,
    is_returned: bool,
}

struct CampingRental {
    gears: HashMap<u64, CampingGear>,
    transactions: Vec<RentalTransaction>,
    next_gear_id: u64,
    admin: Principal,
}

impl CampingRental {
    fn new(admin: Principal) -> Self {
        CampingRental {
            gears: HashMap::new(),
            transactions: Vec::new(),
            next_gear_id: 1,
            admin,
        }
    }

    fn add_gear(&mut self, name: String, description: String, daily_rate: u64, quantity: u64) -> u64 {
        let gear = CampingGear {
            id: self.next_gear_id,
            name,
            description,
            daily_rate,
            quantity,
        };
        self.gears.insert(self.next_gear_id, gear);
        self.next_gear_id += 1;
        self.next_gear_id - 1
    }

    fn list_gears(&self) -> Vec<CampingGear> {
        self.gears.values().cloned().collect()
    }

    fn rent_gear(&mut self, user: Principal, gear_id: u64, days: u64) -> Result<(), String> {
        let gear = self.gears.get_mut(&gear_id).ok_or("Alat camping tidak ditemukan")?;

        if gear.quantity < 1 {
            return Err("Stok alat camping habis".to_string());
        }

        gear.quantity -= 1;
        let total_cost = gear.daily_rate * days;

        let transaction = RentalTransaction {
            user,
            gear_id,
            days,
            total_cost,
            is_returned: false,
        };

        self.transactions.push(transaction);

        Ok(())
    }

    fn list_transactions(&self) -> Vec<RentalTransaction> {
        self.transactions.clone()
    }
}

static mut CAMPING_RENTAL: Option<CampingRental> = None;

#[ic_cdk::init]
fn init(admin: Principal) {
    unsafe {
        CAMPING_RENTAL = Some(CampingRental::new(admin));
    }
}

#[ic_cdk::query]
fn list_gears() -> Vec<CampingGear> {
    let camping_rental = unsafe { CAMPING_RENTAL.as_ref().unwrap() };
    camping_rental.list_gears()
}

#[ic_cdk::update]
fn add_gear(name: String, description: String, daily_rate: u64, quantity: u64) -> u64 {
    let caller = ic_cdk::caller();
    let camping_rental = unsafe { CAMPING_RENTAL.as_mut().unwrap() };

    if caller != camping_rental.admin {
        trap("Hanya admin yang dapat menambahkan alat camping");
    }

    camping_rental.add_gear(name, description, daily_rate, quantity)
}

#[ic_cdk::update]
fn rent_gear(gear_id: u64, days: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();
    let camping_rental = unsafe { CAMPING_RENTAL.as_mut().unwrap() };
    camping_rental.rent_gear(caller, gear_id, days)
}

#[ic_cdk::query]
fn list_transactions() -> Vec<RentalTransaction> {
    let camping_rental = unsafe { CAMPING_RENTAL.as_ref().unwrap() };
    camping_rental.list_transactions()
}
