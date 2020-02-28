/**
 * Gift Wrapping Algorithm By P5.js
 */

var points = [];
var javis = new Javis();

function setup() {
  let width = 400;
  let height = 400;
  let space = 30;
  createCanvas(width, height);
  for (let i = 0; i < 30; i++) {
    let x = random(space, width - space);
    let y = random(space, width - space);
    points[i] = new Point(x, y);
  }
  javis.getLowestPoint(points);
}

function draw() {
  // 終了判定
  if (this.is_completed || javis.contour.length > 30) return;
  background(220);
  for (let i = 0; i < points.length; i++) {
    points[i].show();
  }
  javis.search(points);
  javis.showContour();

}


function Javis() {
  this.contour = [];
  this.is_completed = false;


  this.getLowestPoint = function (points) {
    let ymin = -100000;
    let p = -1;
    for (let i = 0; i < points.length; i++) {
      if (ymin < points[i].y) {
        p = i;
        ymin = points[i].y;
      }
    }
    this.contour.push(points[p]);
  }

  this.search = (points) => {
    let last = this.contour[this.contour.length - 1];
    let max_theta = -10000;
    let max_p = -1;

    for (let i = 0; i < points.length; i++) {

      let theta = this.dot(points[i]);
      console.log(theta);
      if (max_theta < theta && theta != 0) {
        max_theta = theta;
        max_p = i;
      }
    }
    console.log(max_p, max_theta);
    this.contour.push(points[max_p]);
  }

  this.showContour = function () {
    console.log(this.contour.length);
    console.log(this.contour);
    for (let i = 1; i < this.contour.length; i++) {
      this.line(this.contour[i - 1], this.contour[i]);
    }
  }

  this.line = function (a, b) {
    stroke(126);
    line(a.x, a.y, b.x, b.y)
  }

  this.dot = function (point) {
    let len = this.contour.length;
    let last = this.contour[len - 1];
    if (this.contour.length < 2) {
      return -atan2(point.y - last.y, point.x - last.x);
    }

    if (this.contour[len - 2].x - point.x == 0 || this.contour[len - 2].y - point.y == 0) {
      return -111000;
    }

    let vec1 = new Point(point.x - last.x, point.y - last.y);
    let vec2 = new Point(this.contour[len - 2].x - last.x, this.contour[len - 2].y - last.y);

    let cos_theta = (vec1.x * vec2.x + vec1.y * vec2.y) / (sqrt(vec1.x ** 2 + vec1.y ** 2) * sqrt(vec2.x ** 2 + vec2.y ** 2));
    return acos(cos_theta);
  }
}


class Point {
  constructor(x, y) {
    this.x = x;
    this.y = y;
    this.r = 5;
  }

  show() {
    noStroke()
    fill('purple');
    ellipse(this.x, this.y, this.r);
  }
}