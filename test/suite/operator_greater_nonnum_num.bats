# bats file_tags=tag:operator
@test "operator/greater_nonnum_num.lox" {
  run target/debug/lox test/cases/operator/greater_nonnum_num.lox

}
