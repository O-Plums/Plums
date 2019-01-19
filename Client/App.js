import React from 'react';
import { Router, Scene, Stack } from 'react-native-router-flux'
import { AppLoading, Font } from 'expo'
import { Provider } from '@ant-design/react-native'


import Home from './src/views/Home/index.js'
import Conversation from './src/views/Conversation/index.js'

export default class App extends React.Component {

  state = {
    isReady: false,
  };

  async componentDidMount() {
    await Font.loadAsync(
      'antoutline',
      // eslint-disable-next-line
      require('@ant-design/icons-react-native/fonts/antoutline.ttf')
    );

    await Font.loadAsync(
      'antfill',
      // eslint-disable-next-line
      require('@ant-design/icons-react-native/fonts/antfill.ttf')
    );
    // eslint-disable-next-line
    this.setState({ isReady: true });
  }

  render() {
    if (!this.state.isReady) {
      return <AppLoading />;
    }

    return (
       <Provider>
      <Router>
        <Stack key="root">
          <Scene key="home" component={Home} title="Home"/>
          <Scene key="conversation" component={Conversation} title="Conversation"/>
        </Stack>
      </Router>
       </Provider>
    )
  }
}
