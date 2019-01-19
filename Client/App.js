import React from 'react';
import { StyleSheet, Text, View } from 'react-native'

import { AppLoading, Font } from 'expo';

import Home from './Views/Home/index.js'

export default class App extends React.Component {

  state = {
     theme: null,
     currentTheme: null,
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
      <View style={styles.container}>
      <Home />
      </View>
    );
  }
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
  },
});
