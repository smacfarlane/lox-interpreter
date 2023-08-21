# bats file_tags=tag:closure
@test "closure/shadow_closure_with_local.lox" {
  run target/debug/lox test/cases/closure/shadow_closure_with_local.lox

  [ "${lines[0]}" = "closure" ]
  [ "${lines[1]}" = "shadow" ]
  [ "${lines[2]}" = "closure" ]
}
