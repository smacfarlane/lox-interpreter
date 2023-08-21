# bats file_tags=tag:operator
@test "operator/less_nonnum_num.lox" {
  run target/debug/lox test/cases/operator/less_nonnum_num.lox

}
