import React from 'react'
import { StyleSheet, Text, View, ScrollView, KeyboardAvoidingView } from 'react-native'
import { Button, InputItem, Radio, Icon, Flex, WhiteSpace } from '@ant-design/react-native'

import { Actions } from 'react-native-router-flux'

// TOTO je pense que ajouter "redux-saga" peut etre un plus

export default class Conversation extends React.Component {

  state = {
    /* This value or only for Dev  delllette when there not need */
    msg: '',
    currentName: 'blue',
    currentId: 'me',
    tmpMsg: [{ msg: 'Hello', senderId: 'me', time: 1547982318000, name: 'blue' },
      { msg: 'sa va bien ?', senderId: 'him1', time: 1547982123000, name: 'rose' },
      { msg: 'moi sava bien je suis entrain de parler politique avec mes parent.', senderId: 'me', time: 1547982135000, name: 'blue' }],
  }
  sendNewMessage = () => {
    const { tmpMsg, currentName, msg, currentId } = this.state
    if (msg === '') {
      return
    }
    tmpMsg.push({ msg, senderId: currentId, time: new Date(), name: currentName })
    this.setState({
      tmpMsg,
      msg: '',
    })
  }

  render () {
    const { tmpMsg, currentId } = this.state
    return (
      <KeyboardAvoidingView style={styles.container} behavior="padding">
        <View>
        { /* THis need to be a components  need to add date and name */ }
          {tmpMsg && tmpMsg.length > 0 && (
            tmpMsg.map((obj, key) => {
              return (
                <View key={key} style={obj.senderId === currentId ? leftMsg.container : rightMsg.container}>
                  <Text style={{ textAlign: obj.senderId !== currentId ? 'right' : 'left' }}>{obj.msg}</Text>
                </View>
              )
            })
          )}
        </View>
        {/* This need to be a components */}
        <Flex direction='row' style={{ marginBottom: 20, width: '100%', padding: 5 }}>
          <Flex.Item >
            <InputItem
              clear
              value={this.state.msg}
              onChange={value => { this.setState({ msg: value }) }}
              placeholder='new message'
            />
          </Flex.Item>
          <Button type='primary' onPress={this.sendNewMessage} >Send</Button>

        </Flex>
      <View style={{ height: 60 }} />
      </KeyboardAvoidingView>
    )
  }
}

const rightMsg = StyleSheet.create({
  container: {
    marginBottom: 10,
    borderRadius: 10,
    borderWidth: 1,
    borderColor: '#ff93ac',
    padding: 5,
    width: '95%',
    color: '#ff93ac',
    alignSelf: 'center',
    textAlign: 'right',
  },
})
const leftMsg = StyleSheet.create({
  container: {
    marginBottom: 10,
    borderRadius: 10,
    borderWidth: 1,
    borderColor: '#1890ff',
    padding: 5,
    width: '95%',
    color: '#1890ff',
    alignSelf: 'center',
    textAlign: 'left',
  },
})

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    justifyContent: 'flex-end',
  },
})
