import React from 'react';
import { StyleSheet, Text, View } from 'react-native';
import { Button, InputItem, Radio, Icon  } from '@ant-design/react-native';

export default class Conversation extends React.Component {
  state = {
    isSad:true,
  }
  render() {
    return (
      <View style={styles.container}>
        <Text>Hello</Text>
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
