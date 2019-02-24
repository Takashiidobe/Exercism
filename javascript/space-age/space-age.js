const age = (planet, seconds) => {
  const earthTime = 31557600;
  const mercuryTime = .2408467;
  const venusTime = .61519726;
  const marsTime = 1.8808158;
  const jupiterTime = 11.862615;
  const saturnTime = 29.447498;
  const uranusTime = 84.016846;
  const neptuneTime = 164.79132;
  switch (planet) {
    case 'earth':
      return parseInt((seconds / earthTime).toFixed(2));
    case 'mercury':
      return parseInt((seconds / earthTime / mercuryTime).toFixed(2));
    case 'venus':
      return parseInt(seconds / earthTime / venusTime).toFixed(2);
    case 'mars':
      return parseInt(seconds / earthTime / marsTime).toFixed(2);
    case 'jupiter':
      return parseInt(seconds / earthTime / jupiterTime).toFixed(2);
    case 'saturn':
      return parseInt(seconds / earthTime / saturnTime).toFixed(2);
    case 'uranus':
      return parseInt(seconds / earthTime / uranusTime).toFixed(2);
    case 'neptune':
      return parseInt(seconds / earthTime / neptuneTime).toFixed(2);
  }
}


module.exports = {
  age
};

// describe('Space Age', () => {
//   test('age on Earth', () => {
//     expect(age('earth', 1000000000)).toEqual(31.69);
//   });

//   xtest('age on Mercury', () => {
//     expect(age('mercury', 2134835688)).toEqual(280.88);
//   });