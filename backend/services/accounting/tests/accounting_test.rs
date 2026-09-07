use rust_decimal::Decimal;

#[test]
fn test_double_entry_invariant() {
    let debit1 = Decimal::new(10050, 2); // 100.50
    let debit2 = Decimal::new(5000, 2);  // 50.00

    let credit1 = Decimal::new(15050, 2); // 150.50

    let total_debits = debit1 + debit2;
    let total_credits = credit1;

    assert_eq!(total_debits, total_credits, "Total debits must equal total credits");
}

#[test]
fn test_decimal_money_precision() {
    let money = Decimal::new(1834, 2); // 18.34
    assert_eq!(money.to_string(), "18.34");
}
