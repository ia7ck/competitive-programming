// https://img.atcoder.jp/file/language-update/2025-10/language-list.html
val scala3Version = "3.7.2"

lazy val root = project
  .in(file("."))
  .settings(
    name := "abc445-scala",
    version := "0.1.0-SNAPSHOT",
    scalaVersion := scala3Version,
    run / fork := true,
    run / connectInput := true
    // , libraryDependencies += "org.scalameta" %% "munit" % "1.0.0" % Test
  )
