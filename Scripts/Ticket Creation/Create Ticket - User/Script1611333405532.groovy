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

WebUI.click(findTestObject('Create Ticket/User/h1_Create Ticket'))

WebUI.click(findTestObject('Create Ticket/p_A - Katalon Testing'))

WebUI.click(findTestObject('Create Ticket/User/p_Low'))

WebUI.setText(findTestObject('Create Ticket/User/input_Summary'), 'Automated Testing - User')

//WebUI.sendKeys(findTestObject('Create Ticket/User/input_Summary'), Keys.chord('Automated Testing - User', Keys.TAB))
WebUI.setText(findTestObject('Create Ticket/User/RTF/div_Insert Horizontal Line_fr-element fr-view'), 'Automated ticket')

WebUI.click(findTestObject('Create Ticket/User/RTF/svg_Next'))

WebUI.click(findTestObject('Create Ticket/User/RTF/svg_Next2'))

WebUI.click(findTestObject('Create Ticket/User/RTF/svg_Submit'))

WebUI.delay(2)

WebUI.click(findTestObject('Create Ticket/User/span_Close'))

