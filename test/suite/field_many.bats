# bats file_tags=tag:field
skip
@test "field/many.lox" {
  run target/debug/lox test/cases/field/many.lox

  [ "${lines[0]}" = "apple" ]
  [ "${lines[1]}" = "apricot" ]
  [ "${lines[2]}" = "avocado" ]
  [ "${lines[3]}" = "banana" ]
  [ "${lines[4]}" = "bilberry" ]
  [ "${lines[5]}" = "blackberry" ]
  [ "${lines[6]}" = "blackcurrant" ]
  [ "${lines[7]}" = "blueberry" ]
  [ "${lines[8]}" = "boysenberry" ]
  [ "${lines[9]}" = "cantaloupe" ]
  [ "${lines[10]}" = "cherimoya" ]
  [ "${lines[11]}" = "cherry" ]
  [ "${lines[12]}" = "clementine" ]
  [ "${lines[13]}" = "cloudberry" ]
  [ "${lines[14]}" = "coconut" ]
  [ "${lines[15]}" = "cranberry" ]
  [ "${lines[16]}" = "currant" ]
  [ "${lines[17]}" = "damson" ]
  [ "${lines[18]}" = "date" ]
  [ "${lines[19]}" = "dragonfruit" ]
  [ "${lines[20]}" = "durian" ]
  [ "${lines[21]}" = "elderberry" ]
  [ "${lines[22]}" = "feijoa" ]
  [ "${lines[23]}" = "fig" ]
  [ "${lines[24]}" = "gooseberry" ]
  [ "${lines[25]}" = "grape" ]
  [ "${lines[26]}" = "grapefruit" ]
  [ "${lines[27]}" = "guava" ]
  [ "${lines[28]}" = "honeydew" ]
  [ "${lines[29]}" = "huckleberry" ]
  [ "${lines[30]}" = "jabuticaba" ]
  [ "${lines[31]}" = "jackfruit" ]
  [ "${lines[32]}" = "jambul" ]
  [ "${lines[33]}" = "jujube" ]
  [ "${lines[34]}" = "juniper" ]
  [ "${lines[35]}" = "kiwifruit" ]
  [ "${lines[36]}" = "kumquat" ]
  [ "${lines[37]}" = "lemon" ]
  [ "${lines[38]}" = "lime" ]
  [ "${lines[39]}" = "longan" ]
  [ "${lines[40]}" = "loquat" ]
  [ "${lines[41]}" = "lychee" ]
  [ "${lines[42]}" = "mandarine" ]
  [ "${lines[43]}" = "mango" ]
  [ "${lines[44]}" = "marionberry" ]
  [ "${lines[45]}" = "melon" ]
  [ "${lines[46]}" = "miracle" ]
  [ "${lines[47]}" = "mulberry" ]
  [ "${lines[48]}" = "nance" ]
  [ "${lines[49]}" = "nectarine" ]
  [ "${lines[50]}" = "olive" ]
  [ "${lines[51]}" = "orange" ]
  [ "${lines[52]}" = "papaya" ]
  [ "${lines[53]}" = "passionfruit" ]
  [ "${lines[54]}" = "peach" ]
  [ "${lines[55]}" = "pear" ]
  [ "${lines[56]}" = "persimmon" ]
  [ "${lines[57]}" = "physalis" ]
  [ "${lines[58]}" = "pineapple" ]
  [ "${lines[59]}" = "plantain" ]
  [ "${lines[60]}" = "plum" ]
  [ "${lines[61]}" = "plumcot" ]
  [ "${lines[62]}" = "pomegranate" ]
  [ "${lines[63]}" = "pomelo" ]
  [ "${lines[64]}" = "quince" ]
  [ "${lines[65]}" = "raisin" ]
  [ "${lines[66]}" = "rambutan" ]
  [ "${lines[67]}" = "raspberry" ]
  [ "${lines[68]}" = "redcurrant" ]
  [ "${lines[69]}" = "salak" ]
  [ "${lines[70]}" = "salmonberry" ]
  [ "${lines[71]}" = "satsuma" ]
  [ "${lines[72]}" = "strawberry" ]
  [ "${lines[73]}" = "tamarillo" ]
  [ "${lines[74]}" = "tamarind" ]
  [ "${lines[75]}" = "tangerine" ]
  [ "${lines[76]}" = "tomato" ]
  [ "${lines[77]}" = "watermelon" ]
  [ "${lines[78]}" = "yuzu" ]
}
