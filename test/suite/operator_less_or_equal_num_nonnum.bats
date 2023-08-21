# bats file_tags=tag:operator
@test "operator/less_or_equal_num_nonnum.lox" {
  run target/debug/lox test/cases/operator/less_or_equal_num_nonnum.lox

}
