fn main() {
    // Using "&&" check if multiple independent condition must be match (and)
    let _purchased_ticket = true;
    let _plane_on_time = true;
    let _making_event = _purchased_ticket && _plane_on_time;
    println!("It is {} that I will arrive as expected", _making_event);


    // Using "||" check if one of the value is true (or)
    let _user_has_paid_for_subscription = true;
    let __user_is_admin = true;
    let _user_can_see_premium_experience = _user_has_paid_for_subscription || __user_is_admin;
    println!("Can this user see my site? {_user_can_see_premium_experience}");
}
