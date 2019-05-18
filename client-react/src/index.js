import React from 'react';
import ReactDOM from 'react-dom';

import './index.css';
import './index.html';

class GameDisplay extends React.Component {
  constructor(props) {
    console.log("GameDisplay.constructor");
    super(props);
    this.state = { data: null };
  }

  componentDidMount() {
    console.log("GameDisplay.componentDidMount");
    setInterval( () => {this.reload()},1000);
  }

  componentWillUnmount() {
    console.log("GameDisplay.componentWillUnmount");
    clearInterval(this.interval);
  }

  componentDidUpdate() {
    console.log("GameDisplay.componentDidUpdate");
    this.updateCanvas();
  }

  reload() {
    console.log("GameDisplay.reload");
    fetch('http://lvh.me:6060/world/get')
        .then(response => {
          const result = response.json();
          console.log("Result: >>>>> ", result);
          return result;
        })
        .then(data => this.setState({
          width: data.width,
          height: data.height,
          data: data.data
        }))
        .catch(function(error) {
          console.log('Error: >>>', error);
        });
  }

  updateCanvas() {
    console.log("GameDisplay.updateCanvas");
    const width = parseInt(this.state.width, 10);
    const height = parseInt(this.state.height, 10);
    const dataIn = this.state.data;

    const cellWidth = 10;
    const cellHeight = 10;

    let imageWidth = width*cellWidth;
    let imageHeight = height*cellHeight;

    //console.log("DEBUG: width: ", width, "height: ", height, "dataIn: ", dataIn);

    const ctx = this.refs.canvas.getContext('2d');
    let imageData = ctx.getImageData(0,0, imageWidth, imageHeight);
    let dataOut = imageData.data;

    let indexOut = 0;

    for (let indexIn = 0; indexIn < dataOut.length; indexIn ++) {
      if (dataIn[indexIn] !== '\n') {
        const colors = getColorIndicesForSymbol(dataIn[indexIn]);

        for (let dx = 0; dx < cellWidth; dx ++) {
          for (let dy = 0; dy < cellHeight; dy ++) {
            let cellIndex = indexOut + 4 * dx + 4 * imageWidth * dy;
            imageData.data[cellIndex] = colors[0]; // red
            imageData.data[cellIndex + 1] = colors[1]; // green
            imageData.data[cellIndex + 2] = colors[2]; // blue
            imageData.data[cellIndex + 3] = colors[3]; // alpha
          }
        }

        indexOut += 4 * cellWidth;
      } else {
        indexOut += 4 * imageWidth * (cellHeight - 1);
      }
    }

    ctx.putImageData(imageData, 0, 0);

    console.log("Done");
  }

  render() {
    return (
        <div>
          <h1>Oh, brave new world!</h1>
          <canvas id="canvas_01" ref="canvas" width={400} height={400} className="bordered"/>
        </div>
    );
  }}

// ========================================

ReactDOM.render(<GameDisplay />, document.getElementById('root'));

function getColorIndicesForSymbol(symbol) {
  if (symbol === ' ') { return [255, 255, 255, 255] } // empty space
  if (symbol === '+') { return [100, 100, 100, 255] } // corpse
  if (symbol === '*') { return [100, 200, 100, 255] } // reproduces
  if (symbol === 'x') { return [200, 100, 100, 255] } // attacks
  if (symbol === 'o') { return [200, 200, 100, 255] } // photosynthesys
  if (symbol === '@') { return [200, 100, 200, 255] } // defiles
  if (symbol === '.') { return [100, 200, 200, 255] } // weird

  return [255, 255, 255, 255];
}

