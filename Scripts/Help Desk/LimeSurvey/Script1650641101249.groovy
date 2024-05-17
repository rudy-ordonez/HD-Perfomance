import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import groovy.json.JsonSlurper as JsonSlurper
import groovy.json.JsonOutput as JsonOutput
import groovy.transform.Field as Field
import com.kms.katalon.core.testobject.ConditionType as ConditionType

response = WS.sendRequest(findTestObject('Limesurvey/POST GetSessionKey', [('HHD_API_Url') : GlobalVariable.HHD_API_Url, ('LimeSurvey_Username') : GlobalVariable.LimeSurvey_Username
            , ('LimeSurvey_Password') : GlobalVariable.LimeSurvey_Password]))

def outputText = response.getResponseText()

println(JsonOutput.prettyPrint(outputText))

def slurper = new JsonSlurper()

def result = slurper.parseText(response.getResponseBodyContent())

GlobalVariable.LimeSurvey_token = result.result

prop = WS.sendRequest(findTestObject('Limesurvey/POST GetSurveyProperties', [('HHD_API_Url') : GlobalVariable.HHD_API_Url
            , ('LimeSurvey_Token') : GlobalVariable.LimeSurvey_token, ('variable_0') : '', ('variable_1') : '']))

def prop2 = prop.getResponseText()

println(JsonOutput.prettyPrint(prop2))

user = WS.sendRequest(findTestObject('Limesurvey/POST CreateUser', [('HHD_API_Url') : GlobalVariable.HHD_API_Url, ('LimeSurvey_token') : GlobalVariable.LimeSurvey_token]))

def user = user.getResponseText()

println(JsonOutput.prettyPrint(user))

survey = WS.sendRequest(findTestObject('Limesurvey/POST ListSurveys', [('HHD_API_Url') : GlobalVariable.HHD_API_Url, ('LimeSurvey_token') : GlobalVariable.LimeSurvey_token]))

def survey = survey.getResponseText()

println(JsonOutput.prettyPrint(survey))

