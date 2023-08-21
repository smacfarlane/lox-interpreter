# bats file_tags=tag:field
skip
@test "field/call_nonfunction_field.lox" {
  run target/debug/lox test/cases/field/call_nonfunction_field.lox

}
