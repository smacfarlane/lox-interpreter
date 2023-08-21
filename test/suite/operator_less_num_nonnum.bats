# bats file_tags=tag:operator
@test "operator/less_num_nonnum.lox" {
  run target/debug/lox test/cases/operator/less_num_nonnum.lox

}
