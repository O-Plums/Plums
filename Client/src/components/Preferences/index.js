import React from 'react'
import { StyleSheet, Text, View } from 'react-native'
import { Button, InputItem, Radio, Icon, Toast, Modal, Flex, List } from '@ant-design/react-native'
import { Actions } from 'react-native-router-flux'

import Swiper from 'react-native-swiper';

export default class Preferences extends React.Component {
  state = {
    themes:               ["Environment", "Politics", "Economy", "Space", "Technologie"],
    genders:              ["Woman", "Man"],
    room_numbers:         [2, 3, 5],
    selected_theme:       "Environment",
    selected_gender:      "Woman",
    selected_room_number: 2,
  }
  render () {
    return (
      <View style={styles.container}>

        <Swiper loop={false} style={styles.wrapper} onIndexChanged={(index) => this.setState({'selected_room_number': this.state.room_numbers[index]})}>
          {this.state.room_numbers.map((item, key) => {
            return (
              <View key={key} style={styles.slide1}>
                <Flex direction='row'>
                  <Icon style={{ fontSize: 50, color: '#fff', }} name='team' />
                  <Text style={styles.text}>{item}</Text>
                </Flex>
              </View>
            )
          })}
        </Swiper>

        <Swiper loop={false} style={styles.wrapper} onIndexChanged={(index) => this.setState({'selected_gender': this.state.genders[index]})}>
          {this.state.genders.map((item, key) => {
            return (
              <View key={key} style={styles.slide2}>
                <Flex direction='row'>
                  <Text style={styles.text}>{item}</Text>
                </Flex>
              </View>
            )
          })}
        </Swiper>

        <Swiper loop={false} style={styles.wrapper} onIndexChanged={(index) => this.setState({'selected_theme': this.state.themes[index]})}>
          {this.state.themes.map((item, key) => {
            return (
              <View key={key} style={styles.slide3}>
                <Flex direction='row'>
                  <Text style={styles.text}>{item}</Text>
                </Flex>
              </View>
            )
          })}
        </Swiper>

        <Button type='primary'  style={{ marginTop: 50, width: '100%' }}>Join in</Button>
      </View>
    )
  }
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
  },
  wrapper:{
  },
  slide1: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#34495e',
  },
  slide2: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#3498db',
  },
  slide3: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#1abc9c',
  },
  text: {
    color: '#fff',
    fontSize: 30,
    fontWeight: 'bold',
  }
})
