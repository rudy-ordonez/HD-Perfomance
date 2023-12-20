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

WebUI.click(findTestObject('Create Ticket/a_Create Ticket'))

WebUI.click(findTestObject('Create Ticket/p_A - Katalon Testing'))

WebUI.delay(4)

WebUI.setText(findTestObject('Create Ticket/Summary'), 'Automated Testing')

WebUI.setText(findTestObject('Create Ticket/Description'), 'Katalon automated testing')

WebUI.selectOptionByValue(findTestObject('Create Ticket/Priority'), '4', true)

WebUI.selectOptionByLabel(findTestObject('Create Ticket/Assigned To'), 'Thon Thai', true)

WebUI.scrollToElement(findTestObject('Create Ticket/div_Submitted By Me'), 0)

WebUI.click(findTestObject('Create Ticket/div_Submitted By Me'), FailureHandling.STOP_ON_FAILURE)

WebUI.scrollToElement(findTestObject('Create Ticket/button_Create'), 4)

WebUI.mouseOver(findTestObject('Create Ticket/button_Create'))

WebUI.click(findTestObject('Create Ticket/button_Create'))

