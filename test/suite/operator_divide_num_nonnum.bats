# bats file_tags=tag:operator
@test "operator/divide_num_nonnum.lox" {
  run target/debug/lox test/cases/operator/divide_num_nonnum.lox

}
