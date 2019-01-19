import React from 'react';
import { StyleSheet, Text, View } from 'react-native';
import { Button, InputItem, Radio, Icon, Toast, Modal,  Flex, } from '@ant-design/react-native'
import { Actions } from 'react-native-router-flux'

export default class Home extends React.Component {
  state = {
    isSad:true,
    number: '',
    roomNumber: 2,
  }
  onButtonClick4 = () => {
  Modal.prompt(
    'Check phone number',
    'code',
    password => {
      Actions.conversation()
    },
    'secure-text',
    'defaultValue',
  )
}
  startConv = () => {
    const { number } = this.state

    if (number === '') {
      Toast.fail('Phone number is messing', 2);
      return
    } else {
      this.setState({visible2: true})
      this.onButtonClick4()
      return
    }
  }
  render() {
    return (
      <View style={styles.container}>

      <InputItem
        clear
        error
        value={this.state.number}
        onChange={value => {
          console.log("ici =>", value);
          this.setState({
            number: value,
          });
        }}
        extra=""
        placeholder="phone number"
      >
        +33
      </InputItem>
      <Flex direction="row">
        <Icon style={{fontSize: 50}} name="team" />
         <Text  onPress={() => {  this.setState({roomNumber: 2}) }} style={{fontSize: 20, padding: 20, color: this.state.roomNumber === 2 ? 'green' : 'black'}}>2</Text>
         <Text  onPress={() => {  this.setState({roomNumber: 3}) }} style={{fontSize: 20, padding: 20, color: this.state.roomNumber === 3 ? 'green' : 'black'}}>3</Text>
         <Text  onPress={() => {  this.setState({roomNumber: 5}) }} style={{fontSize: 20, padding: 20, color: this.state.roomNumber === 5 ? 'green' : 'black'}}>5</Text>
       </Flex>
       <Flex direction="row">
        <Icon  onPress={() => {  this.setState({isSad: !this.state.isSad})}} style={{fontSize: 35,padding: 20, color: !this.state.isSad ? 'red' : 'grey'}} name={'smile'}/>
        <Icon onPress={() => {  this.setState({isSad: !this.state.isSad})}} style={{fontSize: 35, padding: 20, color: this.state.isSad ? 'red' : 'grey' }} name={'frown'}/>
       </Flex>
      <Button type="primary" onPress={this.startConv} style={{marginTop: 50}}>Let's Go</Button>
      <Text  onPress={this.onButtonClick4} style={{ marginTop:10, fontSize: 13, textDecorationLine: 'underline'}}> I allrey have a code</Text>

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
