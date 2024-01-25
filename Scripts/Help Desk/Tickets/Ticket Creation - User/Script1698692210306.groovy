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

WebUI.click(findTestObject('Help Desk/Ticket Creation/h1_Create Ticket'))

WebUI.click(findTestObject('Help Desk/Ticket Creation/p_Approval Request'))

WebUI.click(findTestObject('Help Desk/Ticket Creation/div_Proceed Without Sub-Problem Type'))

WebUI.click(findTestObject('Help Desk/Ticket Creation/p_Low'))

WebUI.setText(findTestObject('Help Desk/Ticket Creation/input_Summary_summary'), 'Enter a summary line')

WebUI.setText(findTestObject('Help Desk/Ticket Creation/textarea_Description_description'), 'Enter a description')

WebUI.click(findTestObject('Help Desk/Ticket Creation/svg_browse_svg-inline--fa fa-arrow-circle-r_7808f0'))

WebUI.click(findTestObject('Help Desk/Ticket Creation/div_Next'))

WebUI.click(findTestObject('Help Desk/Ticket Creation/svg_Save number to profile_svg-inline--fa f_89c82a'))

WebUI.click(findTestObject('Help Desk/Ticket Creation/button_Return to Dashboard'))

