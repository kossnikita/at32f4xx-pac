#[doc = "Register `XIP_CMD_W2` reader"]
pub type R = crate::R<XIP_CMD_W2_SPEC>;
#[doc = "Register `XIP_CMD_W2` writer"]
pub type W = crate::W<XIP_CMD_W2_SPEC>;
#[doc = "Field `XIPR_DCNT` reader - XIP read data counter"]
pub type XIPR_DCNT_R = crate::FieldReader;
#[doc = "Field `XIPR_DCNT` writer - XIP read data counter"]
pub type XIPR_DCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `XIPR_TCNT` reader - XIP continue read cycle counter"]
pub type XIPR_TCNT_R = crate::FieldReader;
#[doc = "Field `XIPR_TCNT` writer - XIP continue read cycle counter"]
pub type XIPR_TCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XIPR_SEL` reader - XIP read continue mode select"]
pub type XIPR_SEL_R = crate::BitReader;
#[doc = "Field `XIPR_SEL` writer - XIP read continue mode select"]
pub type XIPR_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XIPW_DCNT` reader - XIP write data counter"]
pub type XIPW_DCNT_R = crate::FieldReader;
#[doc = "Field `XIPW_DCNT` writer - XIP write data counter"]
pub type XIPW_DCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `XIPW_TCNT` reader - XIP continue write cycle counter"]
pub type XIPW_TCNT_R = crate::FieldReader;
#[doc = "Field `XIPW_TCNT` writer - XIP continue write cycle counter"]
pub type XIPW_TCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XIPW_SEL` reader - XIP write continue mode select"]
pub type XIPW_SEL_R = crate::BitReader;
#[doc = "Field `XIPW_SEL` writer - XIP write continue mode select"]
pub type XIPW_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - XIP read data counter"]
    #[inline(always)]
    pub fn xipr_dcnt(&self) -> XIPR_DCNT_R {
        XIPR_DCNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - XIP continue read cycle counter"]
    #[inline(always)]
    pub fn xipr_tcnt(&self) -> XIPR_TCNT_R {
        XIPR_TCNT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - XIP read continue mode select"]
    #[inline(always)]
    pub fn xipr_sel(&self) -> XIPR_SEL_R {
        XIPR_SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - XIP write data counter"]
    #[inline(always)]
    pub fn xipw_dcnt(&self) -> XIPW_DCNT_R {
        XIPW_DCNT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:30 - XIP continue write cycle counter"]
    #[inline(always)]
    pub fn xipw_tcnt(&self) -> XIPW_TCNT_R {
        XIPW_TCNT_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - XIP write continue mode select"]
    #[inline(always)]
    pub fn xipw_sel(&self) -> XIPW_SEL_R {
        XIPW_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XIP_CMD_W2")
            .field("xipr_dcnt", &self.xipr_dcnt())
            .field("xipr_tcnt", &self.xipr_tcnt())
            .field("xipr_sel", &self.xipr_sel())
            .field("xipw_dcnt", &self.xipw_dcnt())
            .field("xipw_tcnt", &self.xipw_tcnt())
            .field("xipw_sel", &self.xipw_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - XIP read data counter"]
    #[inline(always)]
    pub fn xipr_dcnt(&mut self) -> XIPR_DCNT_W<'_, XIP_CMD_W2_SPEC> {
        XIPR_DCNT_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - XIP continue read cycle counter"]
    #[inline(always)]
    pub fn xipr_tcnt(&mut self) -> XIPR_TCNT_W<'_, XIP_CMD_W2_SPEC> {
        XIPR_TCNT_W::new(self, 8)
    }
    #[doc = "Bit 15 - XIP read continue mode select"]
    #[inline(always)]
    pub fn xipr_sel(&mut self) -> XIPR_SEL_W<'_, XIP_CMD_W2_SPEC> {
        XIPR_SEL_W::new(self, 15)
    }
    #[doc = "Bits 16:21 - XIP write data counter"]
    #[inline(always)]
    pub fn xipw_dcnt(&mut self) -> XIPW_DCNT_W<'_, XIP_CMD_W2_SPEC> {
        XIPW_DCNT_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - XIP continue write cycle counter"]
    #[inline(always)]
    pub fn xipw_tcnt(&mut self) -> XIPW_TCNT_W<'_, XIP_CMD_W2_SPEC> {
        XIPW_TCNT_W::new(self, 24)
    }
    #[doc = "Bit 31 - XIP write continue mode select"]
    #[inline(always)]
    pub fn xipw_sel(&mut self) -> XIPW_SEL_W<'_, XIP_CMD_W2_SPEC> {
        XIPW_SEL_W::new(self, 31)
    }
}
#[doc = "XIP command word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`xip_cmd_w2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xip_cmd_w2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XIP_CMD_W2_SPEC;
impl crate::RegisterSpec for XIP_CMD_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xip_cmd_w2::R`](R) reader structure"]
impl crate::Readable for XIP_CMD_W2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xip_cmd_w2::W`](W) writer structure"]
impl crate::Writable for XIP_CMD_W2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XIP_CMD_W2 to value 0"]
impl crate::Resettable for XIP_CMD_W2_SPEC {}
