import React from 'react';
import { StyleSheet, Text, View } from 'react-native';
import { Button, InputItem, Radio, Icon  } from '@ant-design/react-native';

export default class Home extends React.Component {
  state = {
    isSad:true,
  }
  render() {
    return (
      <View style={styles.container}>
        <InputItem
         type="phone"
         placeholder="input your phone"
       >+33</InputItem>
       <Radio
          checked={this.state.isSad}
          onChange={() => {
            this.setState({isSad: !this.state.isSad})

          }}

          style={{ borderWidth: 1, borderColor: '#999', margin: 10 }}
        >
          <Icon style={{fontSize: 50}} name={'smile'}/>
        </Radio>
        <Radio
           checked={!this.state.isSad}
           onChange={() => {
             this.setState({isSad: !this.state.isSad})

           }}
           style={{ borderWidth: 1, borderColor: '#999', margin: 10}}
         >
           <Icon style={{fontSize: 50}} name={'frown'}/>
         </Radio>
       <Button type="primary" style={{marginTop: 50}}>Let's Go</Button>

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
