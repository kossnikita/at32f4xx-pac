#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `CBCTRL` reader - Channel buffer control"]
pub type CBCTRL_R = crate::BitReader;
#[doc = "Field `CBCTRL` writer - Channel buffer control"]
pub type CBCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCFS` reader - Channel control bit refresh select"]
pub type CCFS_R = crate::BitReader;
#[doc = "Field `CCFS` writer - Channel control bit refresh select"]
pub type CCFS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRS` reader - DMA request source"]
pub type DRS_R = crate::BitReader;
#[doc = "Field `DRS` writer - DMA request source"]
pub type DRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTOS` reader - Primary TMR output selection"]
pub type PTOS_R = crate::FieldReader;
#[doc = "Field `PTOS` writer - Primary TMR output selection"]
pub type PTOS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `C1IOS` reader - Channel 1 idle output state"]
pub type C1IOS_R = crate::BitReader;
#[doc = "Field `C1IOS` writer - Channel 1 idle output state"]
pub type C1IOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1CIOS` reader - Channel 1 complementary idle output state"]
pub type C1CIOS_R = crate::BitReader;
#[doc = "Field `C1CIOS` writer - Channel 1 complementary idle output state"]
pub type C1CIOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2IOS` reader - Channel 2 idle output state"]
pub type C2IOS_R = crate::BitReader;
#[doc = "Field `C2IOS` writer - Channel 2 idle output state"]
pub type C2IOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2CIOS` reader - Channel 2 complementary idle output state"]
pub type C2CIOS_R = crate::BitReader;
#[doc = "Field `C2CIOS` writer - Channel 2 complementary idle output state"]
pub type C2CIOS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel buffer control"]
    #[inline(always)]
    pub fn cbctrl(&self) -> CBCTRL_R {
        CBCTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Channel control bit refresh select"]
    #[inline(always)]
    pub fn ccfs(&self) -> CCFS_R {
        CCFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA request source"]
    #[inline(always)]
    pub fn drs(&self) -> DRS_R {
        DRS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Primary TMR output selection"]
    #[inline(always)]
    pub fn ptos(&self) -> PTOS_R {
        PTOS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Channel 1 idle output state"]
    #[inline(always)]
    pub fn c1ios(&self) -> C1IOS_R {
        C1IOS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 complementary idle output state"]
    #[inline(always)]
    pub fn c1cios(&self) -> C1CIOS_R {
        C1CIOS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 idle output state"]
    #[inline(always)]
    pub fn c2ios(&self) -> C2IOS_R {
        C2IOS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 complementary idle output state"]
    #[inline(always)]
    pub fn c2cios(&self) -> C2CIOS_R {
        C2CIOS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("c2cios", &self.c2cios())
            .field("c2ios", &self.c2ios())
            .field("c1cios", &self.c1cios())
            .field("c1ios", &self.c1ios())
            .field("ptos", &self.ptos())
            .field("drs", &self.drs())
            .field("ccfs", &self.ccfs())
            .field("cbctrl", &self.cbctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Channel buffer control"]
    #[inline(always)]
    #[must_use]
    pub fn cbctrl(&mut self) -> CBCTRL_W<CTRL2_SPEC> {
        CBCTRL_W::new(self, 0)
    }
    #[doc = "Bit 2 - Channel control bit refresh select"]
    #[inline(always)]
    #[must_use]
    pub fn ccfs(&mut self) -> CCFS_W<CTRL2_SPEC> {
        CCFS_W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA request source"]
    #[inline(always)]
    #[must_use]
    pub fn drs(&mut self) -> DRS_W<CTRL2_SPEC> {
        DRS_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Primary TMR output selection"]
    #[inline(always)]
    #[must_use]
    pub fn ptos(&mut self) -> PTOS_W<CTRL2_SPEC> {
        PTOS_W::new(self, 4)
    }
    #[doc = "Bit 8 - Channel 1 idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c1ios(&mut self) -> C1IOS_W<CTRL2_SPEC> {
        C1IOS_W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 1 complementary idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c1cios(&mut self) -> C1CIOS_W<CTRL2_SPEC> {
        C1CIOS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c2ios(&mut self) -> C2IOS_W<CTRL2_SPEC> {
        C2IOS_W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 2 complementary idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c2cios(&mut self) -> C2CIOS_W<CTRL2_SPEC> {
        C2CIOS_W::new(self, 11)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0;
}
