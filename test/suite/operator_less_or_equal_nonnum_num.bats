# bats file_tags=tag:operator
@test "operator/less_or_equal_nonnum_num.lox" {
  run target/debug/lox test/cases/operator/less_or_equal_nonnum_num.lox

}
