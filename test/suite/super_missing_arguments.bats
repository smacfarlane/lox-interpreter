# bats file_tags=tag:super
skip
@test "super/missing_arguments.lox" {
  run target/debug/lox test/cases/super/missing_arguments.lox

}
