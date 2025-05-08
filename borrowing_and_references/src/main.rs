// Borrowing and references
// Safety and Performance

// References enables to borrow values wothout taking ownership

// Mutable & Immutable

// fn main() {
//     let mut x: i32 = 5;
//     let r: &mut i32 = &mut x;
//     *r += 1;

//     // println!("Value of X is {}.", x);
//     // error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable

//     println!("Value of R is {}.", r);
// }

fn main() {
    let mut account = BankAccount{
        owner: "Alice".to_string(),
        balance: 250.57
    };

    // immutable borrow to check balance
    account.check_balance();

    // Mutable borrow to withdraw money
    account.withdraw(50.15);

    // check balance after withdrawal
    account.check_balance();

}

struct BankAccount {
    owner: String,
    balance: f64
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from the account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}