
calcZoomLevel();
function calcZoomLevel() {
    let lats = myAreas.map((ele) => { return ele.latitude });
    let lngs = myAreas.map((ele) => { return ele.longitude });
    console.log(lats, lngs);
    let lat_diff = calcDiff(lats);
    let lng_diff = calcDiff(lngs);
    console.log(lat_diff, lng_diff);
}

function calcDiff(array) {
    let max = array.reduce((a, b) => a > b ? a : b);
    let min = array.reduce((a, b) => a < b ? a : b);
    return max - min;
}

function latToKm(array) {
    let pole_radius = 6378.137; // km
    let lat = 35; // 日本・北緯35
    lat_degree = (360 * 1000) / (2 * Math.PI * pole_radius)
    let max = array.reduce((a, b) => a > b ? a : b);
    let min = array.reduce((a, b) => a < b ? a : b);
    return max - min;
}
