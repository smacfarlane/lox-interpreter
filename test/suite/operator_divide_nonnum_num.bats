# bats file_tags=tag:operator
@test "operator/divide_nonnum_num.lox" {
  run target/debug/lox test/cases/operator/divide_nonnum_num.lox

}
