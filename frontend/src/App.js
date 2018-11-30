import React, { Component } from 'react';
import Counter from './Counter';
import './App.css';

class App extends Component {

  constructor(props) {
    super(props); 
    this.state = {
      count: -1
    }
  }

  componentDidMount() {
    this.getCount(); 
  }
  
  render() {
    return <div className="App" onClick={this.incrementCount.bind(this)}>
      <Counter count={this.state.count} />
    </div>
  }

  incrementCount() {
    fetch("http://localhost:8080/increment", {
      method : "post"
    }); 

    this.getCount(); 
  }


  getCount() {
    fetch("http://localhost:8080/retrieve")
      .then((response) => {
        response.text().then((text) => {
          this.setState({
            count: text
          }); 
        }) 
      });     
  }

}

export default App;
