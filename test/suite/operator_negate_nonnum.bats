# bats file_tags=tag:operator
@test "operator/negate_nonnum.lox" {
  run target/debug/lox test/cases/operator/negate_nonnum.lox

}
