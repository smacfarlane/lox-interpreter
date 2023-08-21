# bats file_tags=tag:operator
@test "operator/subtract_nonnum_num.lox" {
  run target/debug/lox test/cases/operator/subtract_nonnum_num.lox

}
