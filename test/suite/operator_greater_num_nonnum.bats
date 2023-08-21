# bats file_tags=tag:operator
@test "operator/greater_num_nonnum.lox" {
  run target/debug/lox test/cases/operator/greater_num_nonnum.lox

}
