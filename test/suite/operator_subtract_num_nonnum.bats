# bats file_tags=tag:operator
@test "operator/subtract_num_nonnum.lox" {
  run target/debug/lox test/cases/operator/subtract_num_nonnum.lox

}
