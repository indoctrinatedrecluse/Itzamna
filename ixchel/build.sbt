ThisBuild / scalaVersion := "3.3.4"
ThisBuild / version      := "0.1.0-SNAPSHOT"
ThisBuild / organization := "io.itzamna"

lazy val root = (project in file("."))
  .settings(
    name := "ixchel",
    libraryDependencies ++= Seq(
      "dev.zio" %% "zio"         % "2.1.9",
      "dev.zio" %% "zio-http"    % "3.0.1",
      "dev.zio" %% "zio-json"    % "0.7.3",
      "dev.zio" %% "zio-streams" % "2.1.9"
    ),
    scalacOptions ++= Seq(
      "-deprecation",
      "-feature",
      "-unchecked"
    )
  )
