import sbt.librarymanagement.MavenRepository

name := """dash-board"""
organization := "com.lqxc"

version := "1.2"

lazy val root = (project in file(".")).enablePlugins(PlayScala)
scalaVersion := "2.13.6"
// addSbtPlugin("io.get-coursier" % "sbt-coursier" % "2.0.0-RC6-8")
// addSbtPlugin("com.github.sbt" % "sbt-jni" % "1.5.2")

// resolvers += "Local Maven Repository" at
//   "https://packages.jetbrains.team/maven/p/ij/intellij-dependencies"

libraryDependencies += guice
libraryDependencies += "org.scalatestplus.play" %% "scalatestplus-play" % "5.1.0" % Test
// libraryDependencies += "org.jetbrains.pty4j" % "pty4j" % "0.11.4"
