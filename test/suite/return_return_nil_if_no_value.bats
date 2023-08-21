# bats file_tags=tag:return
@test "return/return_nil_if_no_value.lox" {
  run target/debug/lox test/cases/return/return_nil_if_no_value.lox

  [ "${lines[0]}" = "nil" ]
}
