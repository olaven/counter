import React, {Component} from 'react';

class App extends Component {

    render() {
        return ( 
            <div>
                {this.props.count}
            </div> 
        );
    }

}

export default App;
