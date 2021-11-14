import sbt.librarymanagement.MavenRepository

name := """dash-board"""
organization := "com.lqxc"

version := "1.2"

lazy val root = (project in file(".")).enablePlugins(PlayScala)
scalaVersion := "2.13.6"

libraryDependencies += guice
libraryDependencies += "org.scalatestplus.play" %% "scalatestplus-play" % "5.1.0" % Test
