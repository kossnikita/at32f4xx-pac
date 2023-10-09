#[doc = "Register `XIP_CMD_W1` reader"]
pub type R = crate::R<XIP_CMD_W1_SPEC>;
#[doc = "Register `XIP_CMD_W1` writer"]
pub type W = crate::W<XIP_CMD_W1_SPEC>;
#[doc = "Field `XIPW_DUM2` reader - XIP write second dummy cycle"]
pub type XIPW_DUM2_R = crate::FieldReader;
#[doc = "Field `XIPW_DUM2` writer - XIP write second dummy cycle"]
pub type XIPW_DUM2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `XIPW_OPMODE` reader - XIP write operate mode"]
pub type XIPW_OPMODE_R = crate::FieldReader;
#[doc = "Field `XIPW_OPMODE` writer - XIP write operate mode"]
pub type XIPW_OPMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `XIPW_ADRLEN` reader - XIP write address length"]
pub type XIPW_ADRLEN_R = crate::BitReader;
#[doc = "Field `XIPW_ADRLEN` writer - XIP write address length"]
pub type XIPW_ADRLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XIPW_INSC` reader - XIP write instruction code"]
pub type XIPW_INSC_R = crate::FieldReader;
#[doc = "Field `XIPW_INSC` writer - XIP write instruction code"]
pub type XIPW_INSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - XIP write second dummy cycle"]
    #[inline(always)]
    pub fn xipw_dum2(&self) -> XIPW_DUM2_R {
        XIPW_DUM2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - XIP write operate mode"]
    #[inline(always)]
    pub fn xipw_opmode(&self) -> XIPW_OPMODE_R {
        XIPW_OPMODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - XIP write address length"]
    #[inline(always)]
    pub fn xipw_adrlen(&self) -> XIPW_ADRLEN_R {
        XIPW_ADRLEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:19 - XIP write instruction code"]
    #[inline(always)]
    pub fn xipw_insc(&self) -> XIPW_INSC_R {
        XIPW_INSC_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XIP_CMD_W1")
            .field("xipw_dum2", &format_args!("{}", self.xipw_dum2().bits()))
            .field(
                "xipw_opmode",
                &format_args!("{}", self.xipw_opmode().bits()),
            )
            .field("xipw_adrlen", &format_args!("{}", self.xipw_adrlen().bit()))
            .field("xipw_insc", &format_args!("{}", self.xipw_insc().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<XIP_CMD_W1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - XIP write second dummy cycle"]
    #[inline(always)]
    #[must_use]
    pub fn xipw_dum2(&mut self) -> XIPW_DUM2_W<XIP_CMD_W1_SPEC, 0> {
        XIPW_DUM2_W::new(self)
    }
    #[doc = "Bits 8:10 - XIP write operate mode"]
    #[inline(always)]
    #[must_use]
    pub fn xipw_opmode(&mut self) -> XIPW_OPMODE_W<XIP_CMD_W1_SPEC, 8> {
        XIPW_OPMODE_W::new(self)
    }
    #[doc = "Bit 11 - XIP write address length"]
    #[inline(always)]
    #[must_use]
    pub fn xipw_adrlen(&mut self) -> XIPW_ADRLEN_W<XIP_CMD_W1_SPEC, 11> {
        XIPW_ADRLEN_W::new(self)
    }
    #[doc = "Bits 12:19 - XIP write instruction code"]
    #[inline(always)]
    #[must_use]
    pub fn xipw_insc(&mut self) -> XIPW_INSC_W<XIP_CMD_W1_SPEC, 12> {
        XIPW_INSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "XIP command word 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xip_cmd_w1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xip_cmd_w1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XIP_CMD_W1_SPEC;
impl crate::RegisterSpec for XIP_CMD_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xip_cmd_w1::R`](R) reader structure"]
impl crate::Readable for XIP_CMD_W1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xip_cmd_w1::W`](W) writer structure"]
impl crate::Writable for XIP_CMD_W1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XIP_CMD_W1 to value 0"]
impl crate::Resettable for XIP_CMD_W1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
