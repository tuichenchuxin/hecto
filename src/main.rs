#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
mod editor;
use editor::Editor;

fn main() {
    Editor::new().unwrap().run();
}

// fn calculate_loan_amount() -> f64 {
//     let original_rate = 0.042 / 12.0;
//     let new_rate = 0.039 / 12.0;
//     let total_months = 30 * 12;
//     let monthly_payment_difference = 100.0;
//
//     // Function to calculate monthly payment
//     fn monthly_payment(principal: f64, rate: f64, months: i32) -> f64 {
//         principal * rate * (1.0 + rate).powi(months) / ((1.0 + rate).powi(months) - 1.0)
//     }
//
//     // Use a binary search to find the principal amount
//     let mut low = 0.0;
//     let mut high = 3_000_000.0; // Assume a high enough value
//     let mut mid;
//
//     while high - low > 0.01 {
//         mid = (low + high) / 2.0;
//         let original_payment = monthly_payment(mid, original_rate, total_months);
//         let new_payment = monthly_payment(mid, new_rate, total_months);
//
//         if (original_payment - new_payment - monthly_payment_difference).abs() < 0.01 {
//             return mid;
//         } else if original_payment - new_payment > monthly_payment_difference {
//             high = mid;
//         } else {
//             low = mid;
//         }
//     }
//
//     (low + high) / 2.0
// }
//
// fn main() {
//     let loan_amount = calculate_loan_amount();
//     println!("Remaining loan amount: {:.2}", loan_amount);
// }