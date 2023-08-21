# bats file_tags=tag:method
skip
@test "method/not_found.lox" {
  run target/debug/lox test/cases/method/not_found.lox

}
