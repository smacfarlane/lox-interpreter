# bats file_tags=tag:operator
@test "operator/greater_or_equal_num_nonnum.lox" {
  run target/debug/lox test/cases/operator/greater_or_equal_num_nonnum.lox

}
