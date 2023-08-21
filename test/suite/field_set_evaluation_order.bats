# bats file_tags=tag:field
skip
@test "field/set_evaluation_order.lox" {
  run target/debug/lox test/cases/field/set_evaluation_order.lox

}
