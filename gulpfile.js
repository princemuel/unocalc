import gulp from "gulp";
import gulpSass from "gulp-sass";
import nodeSass from "node-sass";

const sass = gulpSass(nodeSass);

gulp.task("styles", function () {
  return gulp
    .src("./scss/main.scss")
    .pipe(sass())
    .pipe(gulp.dest("./styles/"));
});

gulp.task("serve", function () {
  gulp.watch("./scss/**/*.scss", gulp.series(["styles"]));
});
