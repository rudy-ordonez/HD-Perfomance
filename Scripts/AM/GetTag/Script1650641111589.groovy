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
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import groovy.json.JsonSlurper as JsonSlurper
import groovy.json.JsonOutput as JsonOutput
import groovy.transform.Field as Field
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.webservice.verification.WSResponseManager as WSResponseManager

f = new File('C:/Temp/gettag.csv')

//Records start time
long a1 = System.currentTimeMillis()

tag = WS.sendRequestAndVerify(findTestObject('Postman/Asset Management/Tag/Get Tag', [('TIPWebAPI_Url') : GlobalVariable.TIPWebAPI_Url]))

//Records end time
long a2 = System.currentTimeMillis()

(((f << new Date(a1).format('yyyy-MM-dd HH:mm:ss.SSS')) << ',') << ((a2 - a1) / 1000)) << '\r\n'

def output = tag.getResponseText()

println(JsonOutput.prettyPrint(output))

assertThat(output).contains('tag')

