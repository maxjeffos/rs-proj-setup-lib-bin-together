use rs_proj_setup_lib_bin_together::get_message;
use rs_proj_setup_lib_bin_together::outer_get_num;
use rs_proj_setup_lib_bin_together::thing::inner_get_num;

#[test]
fn test_trivial() {
    assert_eq!(1, 1);
}

#[test]
fn test_get_message() {
    assert_eq!(get_message(), "hello");
}

#[test]
fn test_outer_get_num() {
    assert_eq!(outer_get_num(), 5);
}

#[test]
fn test_inner_get_num() {
    assert_eq!(inner_get_num(), 5);
}
