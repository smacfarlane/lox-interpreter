# bats file_tags=tag:operator
@test "operator/multiply_nonnum_num.lox" {
  run target/debug/lox test/cases/operator/multiply_nonnum_num.lox

}
